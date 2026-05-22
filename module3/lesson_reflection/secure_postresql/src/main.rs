use aes_gcm::aead::rand_core::RngCore;
use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use std::env;
use std::error::Error;
use std::io;
use tokio_postgres::NoTls;

const DEFAULT_ADMIN_DB_URL: &str = "postgres://postgres:postgres@localhost:5432/postgres";
const DEFAULT_APP_DB_URL: &str = "postgres://postgres:postgres@localhost:5432/secure_users";
const DEFAULT_APP_DB_NAME: &str = "secure_users";

struct SampleUser {
	username: &'static str,
	email: &'static str,
	phone: &'static str,
}

struct EncryptedValue {
	nonce: Vec<u8>,
	ciphertext: Vec<u8>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let admin_db_url = env::var("ADMIN_DATABASE_URL").unwrap_or_else(|_| DEFAULT_ADMIN_DB_URL.to_string());
	let app_db_url = env::var("APP_DATABASE_URL").unwrap_or_else(|_| DEFAULT_APP_DB_URL.to_string());
	let app_db_name = env::var("APP_DATABASE_NAME").unwrap_or_else(|_| DEFAULT_APP_DB_NAME.to_string());
	let encryption_key = load_encryption_key()?;

	create_database_if_missing(&admin_db_url, &app_db_name).await?;
	println!("Database ready: {app_db_name}");

	let (client, connection) = tokio_postgres::connect(&app_db_url, NoTls).await?;
	tokio::spawn(async move {
		if let Err(err) = connection.await {
			eprintln!("PostgreSQL connection error: {err}");
		}
	});

	create_secure_table(&client).await?;
	insert_encrypted_users(&client, &encryption_key).await?;
	read_and_decrypt_users(&client, &encryption_key).await?;

	Ok(())
}

fn load_encryption_key() -> Result<[u8; 32], Box<dyn Error>> {
	match env::var("DATA_KEY_HEX") {
		Ok(value) => {
			let bytes = hex::decode(value)?;
			if bytes.len() != 32 {
				return Err("DATA_KEY_HEX must decode to exactly 32 bytes".into());
			}
			let mut key = [0_u8; 32];
			key.copy_from_slice(&bytes);
			Ok(key)
		}
		Err(_) => {
			eprintln!("DATA_KEY_HEX not set. Using built-in demo key (do not use in production).");
			Ok(*b"0123456789ABCDEF0123456789ABCDEF")
		}
	}
}

async fn create_database_if_missing(admin_db_url: &str, db_name: &str) -> Result<(), Box<dyn Error>> {
	let (client, connection) = tokio_postgres::connect(admin_db_url, NoTls).await?;
	tokio::spawn(async move {
		if let Err(err) = connection.await {
			eprintln!("PostgreSQL admin connection error: {err}");
		}
	});

	let existing = client
		.query_opt("SELECT 1 FROM pg_database WHERE datname = $1", &[&db_name])
		.await?;
	if existing.is_none() {
		let escaped_name = db_name.replace('"', "\"\"");
		let create_stmt = format!("CREATE DATABASE \"{escaped_name}\"");
		client.execute(&create_stmt, &[]).await?;
		println!("Created database: {db_name}");
	}

	Ok(())
}

async fn create_secure_table(client: &tokio_postgres::Client) -> Result<(), Box<dyn Error>> {
	client
		.execute(
			"
			CREATE TABLE IF NOT EXISTS secure_users (
				id SERIAL PRIMARY KEY,
				username TEXT NOT NULL UNIQUE,
				email_ciphertext BYTEA NOT NULL,
				email_nonce BYTEA NOT NULL,
				phone_ciphertext BYTEA NOT NULL,
				phone_nonce BYTEA NOT NULL,
				created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
			)
			",
			&[],
		)
		.await?;
	println!("Table ready: secure_users");
	Ok(())
}

async fn insert_encrypted_users(
	client: &tokio_postgres::Client,
	key: &[u8; 32],
) -> Result<(), Box<dyn Error>> {
	let users = [
		SampleUser {
			username: "alice",
			email: "alice@example.com",
			phone: "+1-555-0101",
		},
		SampleUser {
			username: "bob",
			email: "bob@example.com",
			phone: "+1-555-0202",
		},
		SampleUser {
			username: "carol",
			email: "carol@example.com",
			phone: "+1-555-0303",
		},
	];

	for user in &users {
		let encrypted_email = encrypt_value(key, user.email.as_bytes())?;
		let encrypted_phone = encrypt_value(key, user.phone.as_bytes())?;

		client
			.execute(
				"
				INSERT INTO secure_users (
					username, email_ciphertext, email_nonce, phone_ciphertext, phone_nonce
				)
				VALUES ($1, $2, $3, $4, $5)
				ON CONFLICT (username) DO UPDATE
					SET email_ciphertext = EXCLUDED.email_ciphertext,
						email_nonce = EXCLUDED.email_nonce,
						phone_ciphertext = EXCLUDED.phone_ciphertext,
						phone_nonce = EXCLUDED.phone_nonce
				",
				&[
					&user.username,
					&encrypted_email.ciphertext,
					&encrypted_email.nonce,
					&encrypted_phone.ciphertext,
					&encrypted_phone.nonce,
				],
			)
			.await?;
	}

	println!("Inserted/updated {} encrypted user rows", users.len());
	Ok(())
}

async fn read_and_decrypt_users(
	client: &tokio_postgres::Client,
	key: &[u8; 32],
) -> Result<(), Box<dyn Error>> {
	let rows = client
		.query(
			"
			SELECT id, username, email_ciphertext, email_nonce, phone_ciphertext, phone_nonce
			FROM secure_users
			ORDER BY id
			",
			&[],
		)
		.await?;

	println!("Decrypted data:");
	for row in rows {
		let id: i32 = row.get("id");
		let username: String = row.get("username");
		let email_ciphertext: Vec<u8> = row.get("email_ciphertext");
		let email_nonce: Vec<u8> = row.get("email_nonce");
		let phone_ciphertext: Vec<u8> = row.get("phone_ciphertext");
		let phone_nonce: Vec<u8> = row.get("phone_nonce");

		let email = decrypt_value(key, &email_nonce, &email_ciphertext)?;
		let phone = decrypt_value(key, &phone_nonce, &phone_ciphertext)?;

		println!("  id={id}, username={username}, email={email}, phone={phone}");
	}

	Ok(())
}

fn encrypt_value(key: &[u8; 32], plaintext: &[u8]) -> Result<EncryptedValue, Box<dyn Error>> {
	let cipher = Aes256Gcm::new_from_slice(key)
		.map_err(|e| io::Error::other(format!("cipher init failed: {e}")))?;
	let mut nonce_bytes = [0_u8; 12];
	OsRng.fill_bytes(&mut nonce_bytes);
	let nonce = Nonce::from_slice(&nonce_bytes);
	let ciphertext = cipher
		.encrypt(nonce, plaintext)
		.map_err(|e| io::Error::other(format!("encryption failed: {e}")))?;

	Ok(EncryptedValue {
		nonce: nonce_bytes.to_vec(),
		ciphertext,
	})
}

fn decrypt_value(key: &[u8; 32], nonce: &[u8], ciphertext: &[u8]) -> Result<String, Box<dyn Error>> {
	if nonce.len() != 12 {
		return Err("invalid nonce length; expected 12 bytes".into());
	}

	let cipher = Aes256Gcm::new_from_slice(key)
		.map_err(|e| io::Error::other(format!("cipher init failed: {e}")))?;
	let plaintext = cipher
		.decrypt(Nonce::from_slice(nonce), ciphertext)
		.map_err(|e| io::Error::other(format!("decryption failed: {e}")))?;
	Ok(String::from_utf8(plaintext)?)
}

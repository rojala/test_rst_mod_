# Russian Troll Tweet Datasets

https://neo4j.com/news/nbc-news-russian-trolls-tweets/


---

### **Reflection Questions**

1. According to the article, what key insights did analyzing the tweets with Neo4j provide? How could a graph database enhance analysis?

    - **Coordinated behavior** among troll accounts.
    - **Retweet patterns**, **hashtag usage**, and **activity spikes** (e.g., during Moscow business hours).
    - **Impersonation tactics**, where trolls posed as U.S. citizens, local media, and political groups.
    - **Influencer identification** using graph algorithms like **PageRank** and **community detection**.

    **Graph database advantage**: Neo4j’s relationships-first model allowed reporters to visualize and analyze complex connections between tweets, users, hashtags, and applications—something relational databases struggle with.

---

2. According to the article, why were the deleted Russian troll tweets considered essential to analyze? What risks did they aim to highlight?

    - They represented **malicious activity** tied to Russian influence operations.
    - They showed how trolls **masqueraded as legitimate voices** to sway public opinion.
    - Their deletion by Twitter meant traditional analysis was impossible—Neo4j enabled **restoration and pattern detection**.

    The risks highlighted included:
    - **Manipulation of democratic discourse**.
    - **Amplification of divisive narratives**.
    - **Use of plausible deniability** by foreign actors.

---

3. How could analyzing these tweets with Neo4j help identify disinformation patterns and prevent manipulation?

    - **Clusters of coordinated accounts**.
    - **Influence hierarchies** (who originated vs. who amplified).
    - **Temporal spikes** in activity.
    - **Common tactics**, like replying to popular tweets to gain visibility.

    This enables **proactive detection** of manipulation attempts by revealing **network structures** and **behavioral patterns**.

---

4. What capabilities of Neo4j allowed for efficiently analyzing 200,000 complex, connected tweets?

    - **Graph algorithms**: PageRank, community detection, centrality measures.
    - **Cypher query language**: Efficient querying of relationships.
    - **Visualization tools**: Helped journalists explore the data interactively.
    - **Scalability**: Handled large, complex datasets with ease.

---

5. How could graph analysis of social networks help identify coordinated influence campaigns in the future?

    - Detect **bot networks** and **fake personas**.
    - Reveal **cross-platform coordination**.
    - Identify **key influencers** and **content amplifiers**.
    - Track **evolution of narratives** over time.

    Governments and platforms can use this to **intervene early** and **mitigate harm**.

---



Build a small Neo4j graph to model how troll accounts might coordinate influence campaigns on social media.

Prototype a Neo4j pipeline to gather tweets from the Twitter API and ingest them into a graph for analysis.



### **Challenges & Tasks**

1. Research Neo4j's graph algorithms and data science tools for connected data analysis. Summarize the most relevant for analyzing social networks.

    - **PageRank**: Finds influential nodes.
    - **Community Detection**: Identifies clusters of coordinated users.
    - **Betweenness Centrality**: Detects nodes that bridge communities.
    - **Similarity Algorithms**: Finds users with similar behavior.

2. Find tweets or other social network data dataset and develop questions you could analyze with Neo4j.

    - Information Operations Archive (RAND)
        * Content: Over 10 million tweets and Reddit posts from Russian and Iranian state-sponsored campaigns.
        * Use Case: Ideal for graph-based analysis of coordinated behavior across platforms.
        * Access: Free and public. [www.rand.org]
    -  MuMiN Dataset
        * Content: Over 21 million tweets, 1.9 million users, and 12,914 fact-checked claims in 41 languages.
        * Structure: Provided as a heterogeneous graph, perfect for Neo4j.
        * Use Case: Multilingual misinformation detection, claim verification, and tweet classification.
        * Access: Open source with tutorials and baseline models. [mumin-data....github.io]
    - Twitter State Trolls Dataset (Chua Chin Hon)
        * Content: Tweets from Russian, Iranian, and Venezuelan troll accounts, plus real tweets from verified users.
        * Use Case: Training classifiers, graph modeling of troll networks.
        * Access: Public GitHub repo with notebooks and data. [https://github.com/chuachinhon/twitter_state_trolls_cch]
    - AWESOME Misinformation Detection
        * Content: Labeled tweets on COVID-19 misinformation and fake news.
        * Use Case: NLP and classification models, graph-based clustering.
        * Access: Public datasets and code. [https://github.com/cococokoko/AWESOME-misinformation-detection]
    - MisInfoText (SFU Discourse Lab)
        * Content: Fact-checked news articles scraped from Snopes, Buzzfeed, PolitiFact, and Emergent.
        * Use Case: Claim verification, misinformation detection, graph modeling of sources and claims.
        * Access: Public datasets with veracity labels. [https://github.com/sfu-discourse-lab/MisInfoText]

3. Implement a simple graph database in Neo4j and write a Cypher query to analyze node relationships.

    [See src/neo4_cypher.rs](src/neo4_cypher.rs).

    Example Cypher query:
    ```cypher
    MATCH (u:User)-[:POSTED]->(t:Tweet)-[:USED_HASHTAG]->(h:Hashtag)
    RETURN u.name, t.text, h.name
    LIMIT 10
    ```
    Build

        cargo run --features neo4_troll

4. Build a small Neo4j graph to model how troll accounts might coordinate influence campaigns on social media.

    [See src/neo4_troll.rs](src/neo4_troll.rs).

    Build
    
        cargo run --features neo4_troll

5. Prototype a Neo4j pipeline to gather tweets from the Twitter API and ingest them into a graph for analysis.

    Steps:
    1. Use Twitter API to collect tweets.
    2. Parse JSON into nodes and relationships.
    3. Use Neo4j driver (Python, JavaScript, etc.) to ingest data.
    4. Run Cypher queries and graph algorithms.

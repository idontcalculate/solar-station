# Solar-Station Terminal

## Overview

**Solar-Station** is a decentralized, community-building platform designed to connect like-minded eco-activists, solarpunks, and resource-sharers. It operates through a minimalistic, low-bandwidth terminal interface, accessible even in areas with limited internet access. This project provides tools for anonymous communication, direct resource-sharing, and decentralized micro-funding for community-driven, eco-friendly projects.

---

## Features

- **User Profiles**: Users, known as "Solarians," can create profiles with an alias, skill tags, bio, and wallet address for micro-transactions.
- **Project Directories**: Projects seeking support or resources can list details, needs, and funding goals.
- **Geo-location and Map Integration**: Locate nearby Solarians, projects, and community hubs through an interactive map.
- **Vector Database and Knowledge Graphs**: Powered by Qdrant for vector storage and Neo4j for recommendation and community insights.
- **Encrypted Communication**: Private group chats for project discussions and guerilla meetups, fostering a safe space for activism.
- **Decentralized Micro-Funding**: Support eco-friendly projects through anonymous crypto transactions.

---

## Prototype Architecture

### **Backend (Rust)**
- **Framework**: Built using Rust with Axum for API endpoints and Qdrant for vectorized data.
- **Key Modules**:
  - **User Profiles**: `/users` for creating, listing, and connecting with Solarians.
  - **Projects**: `/projects` to view, create, and fund eco-initiatives.
  - **Crypto Wallet**: Enable micro-transactions through integrations with rust-bitcoin and ethers-rs libraries.

### **Frontend (Glitch)**
- **Node.js API**: Simulate frontend interactions with Rust backend through Node routes.
- **TypeScript UI**: Planned integration for a terminal-based user experience using Ratatui for Rust.
- **Glitch Modules**: Connects to Rust backend endpoints for real-time interactions, maps, and resource tracking.

### **Database**
- **Qdrant**: Vector database for recommendation and proximity matching.
- **Neo4j**: Knowledge graph to provide connections between Solarians and projects.

---

## Key Modules for MVP

- **Community Directory**: Profiles with skills, location (optional), and anonymous connections.
- **Project Pages**: Each project includes goals, resources, and contributors.
- **Wallet and Microtransactions**: Enable peer-to-peer support with anonymous crypto contributions.
- **Decentralized, Encrypted Messaging**: Matrix protocol integration for secure group chats.

---

## Contributing
- **Contributions are welcome! To contribute**:

- **Fork the repository**.
- **Make your changes**.
- **Submit a pull request**.

## License
**This project is open-source and licensed under the MIT License**.

**Let’s reshape society with decentralized activism and sustainable living.** 🌍🌱

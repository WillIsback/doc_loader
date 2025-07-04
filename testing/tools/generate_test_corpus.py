# Test Document Collection Generator
# This script creates reliable test documents for the doc_loader project

import json
import csv
from pathlib import Path

def create_technical_document():
    """Create a comprehensive technical document"""
    content = """# Technical Documentation: RESTful API Design Guidelines

## Introduction

This document outlines best practices for designing and implementing RESTful APIs in modern web applications. It covers HTTP methods, status codes, authentication, and data serialization.

## HTTP Methods

### GET Requests
- Used for retrieving data
- Should be idempotent
- No request body
- Cacheable by default

### POST Requests
- Used for creating new resources
- Not idempotent
- Includes request body
- Should return 201 Created on success

### PUT Requests
- Used for updating entire resources
- Idempotent operation
- Replaces entire resource
- Should return 200 OK or 204 No Content

### DELETE Requests
- Used for removing resources
- Idempotent operation
- Should return 204 No Content
- May return 404 if resource doesn't exist

## Status Codes

### 2xx Success
- 200 OK: Request successful
- 201 Created: Resource created
- 204 No Content: Success with no response body

### 4xx Client Error
- 400 Bad Request: Invalid request syntax
- 401 Unauthorized: Authentication required
- 403 Forbidden: Access denied
- 404 Not Found: Resource doesn't exist

### 5xx Server Error
- 500 Internal Server Error: Server-side error
- 502 Bad Gateway: Invalid response from upstream
- 503 Service Unavailable: Server temporarily unavailable

## Authentication

### Bearer Token Authentication
```
Authorization: Bearer <token>
```

### API Key Authentication
```
X-API-Key: <api-key>
```

## Data Serialization

### JSON Format
- Use camelCase for property names
- Include appropriate Content-Type headers
- Validate JSON schema

### XML Format
- Use clear element names
- Include proper namespaces
- Validate against XSD schema

## Conclusion

Following these guidelines ensures consistent, maintainable, and scalable API design that improves developer experience and system reliability.
"""
    return content

def create_business_report():
    """Create a business performance report"""
    content = """QUARTERLY BUSINESS PERFORMANCE REPORT
Q4 2024 - EXECUTIVE SUMMARY

COMPANY OVERVIEW
================

TechCorp Solutions has demonstrated strong performance in Q4 2024, with significant growth across all key metrics. This report provides a comprehensive analysis of our financial performance, operational achievements, and strategic initiatives.

FINANCIAL HIGHLIGHTS
====================

Revenue Performance:
- Total Revenue: $15.2M (+18% YoY)
- Recurring Revenue: $12.1M (+22% YoY)
- New Customer Revenue: $3.1M (+35% YoY)

Profitability Metrics:
- Gross Margin: 68% (+3% QoQ)
- Operating Margin: 15% (+2% QoQ)
- Net Profit Margin: 12% (+1% QoQ)

Cash Flow Analysis:
- Operating Cash Flow: $2.8M (+45% QoQ)
- Free Cash Flow: $2.2M (+52% QoQ)
- Cash Reserves: $8.5M (+12% QoQ)

OPERATIONAL ACHIEVEMENTS
========================

Customer Growth:
- Total Active Customers: 1,247 (+28% YoY)
- Customer Retention Rate: 94% (+2% QoQ)
- Net Promoter Score: 72 (+8 points YoY)

Product Development:
- New Features Delivered: 24
- Bug Fixes Released: 156
- Platform Uptime: 99.8%

Team Expansion:
- Total Employees: 89 (+31% YoY)
- Engineering Team: 34 (+41% YoY)
- Sales Team: 18 (+50% YoY)

STRATEGIC INITIATIVES
=====================

Market Expansion:
- Entered 3 new geographic markets
- Launched enterprise product tier
- Established strategic partnerships

Technology Investment:
- Cloud infrastructure upgrade completed
- AI/ML capabilities enhanced
- Security compliance certifications obtained

CHALLENGES AND RISKS
====================

Market Competition:
- Increased competitive pressure in core markets
- Price competition affecting margin growth
- Need for continued innovation investment

Operational Scaling:
- Talent acquisition challenges
- Infrastructure scaling requirements
- Process standardization needs

OUTLOOK FOR 2025
=================

Growth Targets:
- Revenue Target: $22M (+45% YoY)
- Customer Base: 1,800 (+44% YoY)
- Team Size: 125 (+40% YoY)

Strategic Focus Areas:
- International market expansion
- Product portfolio diversification
- Enterprise customer acquisition
- Operational efficiency improvements

This report demonstrates our strong foundation and positions us well for continued growth in 2025. We remain committed to delivering exceptional value to our customers while building a sustainable, profitable business.

Prepared by: Finance Team
Date: January 15, 2025
Classification: Internal Use Only
"""
    return content

def create_research_papers():
    """Create a collection of research paper metadata"""
    papers = [
        {
            "id": "paper_001",
            "title": "Advances in Natural Language Processing: A Comprehensive Survey",
            "authors": ["Dr. Sarah Chen", "Prof. Michael Rodriguez", "Dr. Emily Watson"],
            "abstract": "This survey provides a comprehensive overview of recent advances in natural language processing, covering transformer architectures, pre-trained language models, and their applications in various domains. We analyze the evolution from rule-based systems to neural approaches, highlighting key innovations and their impact on downstream tasks.",
            "publication_date": "2024-03-15",
            "journal": "Journal of Artificial Intelligence Research",
            "volume": 78,
            "pages": "145-189",
            "doi": "10.1613/jair.1.13892",
            "keywords": ["natural language processing", "transformers", "language models", "neural networks"],
            "citation_count": 142,
            "field": "Computer Science"
        },
        {
            "id": "paper_002", 
            "title": "Climate Change Impact on Biodiversity: A Global Analysis",
            "authors": ["Dr. James Thompson", "Prof. Maria Santos", "Dr. David Park"],
            "abstract": "We present a global analysis of climate change impacts on biodiversity across different ecosystems. Using satellite data and field observations from 2000-2023, we identify significant species distribution shifts and ecosystem disruptions. Our findings suggest accelerated biodiversity loss in tropical regions and alpine environments.",
            "publication_date": "2024-06-22",
            "journal": "Nature Climate Change",
            "volume": 14,
            "pages": "78-95",
            "doi": "10.1038/s41558-024-02087-3",
            "keywords": ["climate change", "biodiversity", "ecosystem", "conservation", "global warming"],
            "citation_count": 89,
            "field": "Environmental Science"
        },
        {
            "id": "paper_003",
            "title": "Quantum Computing Applications in Cryptography: Current State and Future Prospects",
            "authors": ["Dr. Lisa Chang", "Prof. Robert Kim", "Dr. Ahmed Hassan"],
            "abstract": "This paper reviews current applications of quantum computing in cryptography, examining both opportunities and threats. We analyze quantum key distribution protocols, post-quantum cryptographic algorithms, and the timeline for quantum advantage in breaking classical encryption schemes.",
            "publication_date": "2024-09-10",
            "journal": "IEEE Transactions on Quantum Engineering",
            "volume": 5,
            "pages": "1-23",
            "doi": "10.1109/TQE.2024.3456789",
            "keywords": ["quantum computing", "cryptography", "quantum key distribution", "post-quantum", "security"],
            "citation_count": 56,
            "field": "Computer Science"
        },
        {
            "id": "paper_004",
            "title": "Machine Learning for Drug Discovery: Accelerating Pharmaceutical Research",
            "authors": ["Dr. Anna Kowalski", "Prof. Giuseppe Rossi", "Dr. Yuki Tanaka"],
            "abstract": "We investigate the application of machine learning techniques in drug discovery processes, from target identification to clinical trial optimization. Our analysis covers deep learning models for molecular property prediction, generative models for drug design, and AI-driven clinical trial patient selection.",
            "publication_date": "2024-11-05",
            "journal": "Nature Medicine",
            "volume": 30,
            "pages": "234-251",
            "doi": "10.1038/s41591-024-03234-5",
            "keywords": ["machine learning", "drug discovery", "pharmaceutical", "deep learning", "clinical trials"],
            "citation_count": 78,
            "field": "Medicine"
        },
        {
            "id": "paper_005",
            "title": "Sustainable Energy Storage Solutions: A Comparative Analysis",
            "authors": ["Dr. Carlos Mendez", "Prof. Helen Johnson", "Dr. Raj Patel"],
            "abstract": "This study compares various sustainable energy storage technologies including lithium-ion batteries, flow batteries, compressed air energy storage, and hydrogen fuel cells. We evaluate efficiency, cost, scalability, and environmental impact across different applications and deployment scenarios.",
            "publication_date": "2024-08-18",
            "journal": "Energy & Environmental Science",
            "volume": 17,
            "pages": "567-598",
            "doi": "10.1039/D4EE02345A",
            "keywords": ["energy storage", "sustainability", "renewable energy", "batteries", "hydrogen"],
            "citation_count": 103,
            "field": "Energy Engineering"
        }
    ]
    return papers

def create_company_dataset():
    """Create a comprehensive company performance dataset"""
    companies = [
        {
            "company_id": "TECH001",
            "company_name": "InnovateTech Solutions",
            "industry": "Software Development",
            "founded": 2018,
            "employees": 245,
            "headquarters": "San Francisco, CA",
            "annual_revenue_2024": 45000000,
            "annual_revenue_2023": 38000000,
            "revenue_growth": 18.4,
            "profit_margin": 15.2,
            "market_cap": 680000000,
            "stock_price": 127.45,
            "debt_to_equity": 0.23,
            "customer_count": 1847,
            "customer_retention": 94.2,
            "nps_score": 72
        },
        {
            "company_id": "BIO002", 
            "company_name": "BioMedical Innovations",
            "industry": "Biotechnology",
            "founded": 2015,
            "employees": 156,
            "headquarters": "Boston, MA",
            "annual_revenue_2024": 89000000,
            "annual_revenue_2023": 76000000,
            "revenue_growth": 17.1,
            "profit_margin": 22.8,
            "market_cap": 1200000000,
            "stock_price": 78.92,
            "debt_to_equity": 0.18,
            "customer_count": 234,
            "customer_retention": 97.8,
            "nps_score": 85
        },
        {
            "company_id": "FIN003",
            "company_name": "FinTech Dynamics",
            "industry": "Financial Technology",
            "founded": 2019,
            "employees": 89,
            "headquarters": "New York, NY",
            "annual_revenue_2024": 23000000,
            "annual_revenue_2023": 18000000,
            "revenue_growth": 27.8,
            "profit_margin": 12.5,
            "market_cap": 340000000,
            "stock_price": 45.67,
            "debt_to_equity": 0.31,
            "customer_count": 5632,
            "customer_retention": 89.4,
            "nps_score": 68
        },
        {
            "company_id": "ENE004",
            "company_name": "GreenEnergy Corp",
            "industry": "Renewable Energy",
            "founded": 2016,
            "employees": 412,
            "headquarters": "Austin, TX",
            "annual_revenue_2024": 156000000,
            "annual_revenue_2023": 134000000,
            "revenue_growth": 16.4,
            "profit_margin": 18.7,
            "market_cap": 2100000000,
            "stock_price": 156.23,
            "debt_to_equity": 0.42,
            "customer_count": 89234,
            "customer_retention": 91.7,
            "nps_score": 74
        },
        {
            "company_id": "MAN005",
            "company_name": "Advanced Manufacturing Systems",
            "industry": "Industrial Manufacturing",
            "founded": 2012,
            "employees": 1247,
            "headquarters": "Detroit, MI",
            "annual_revenue_2024": 340000000,
            "annual_revenue_2023": 298000000,
            "revenue_growth": 14.1,
            "profit_margin": 9.8,
            "market_cap": 1800000000,
            "stock_price": 89.34,
            "debt_to_equity": 0.56,
            "customer_count": 456,
            "customer_retention": 96.1,
            "nps_score": 79
        }
    ]
    return companies

def create_book_catalog():
    """Create a comprehensive book catalog dataset"""
    books = [
        {
            "isbn": "9780134685991",
            "title": "Effective Java",
            "author": "Joshua Bloch",
            "publisher": "Addison-Wesley Professional",
            "publication_year": 2018,
            "pages": 416,
            "genre": "Programming",
            "language": "English",
            "price": 54.99,
            "rating": 4.8,
            "reviews_count": 1247,
            "format": "Hardcover",
            "weight_kg": 0.73,
            "dimensions": "23.5 x 18.7 x 2.8 cm",
            "availability": "In Stock"
        },
        {
            "isbn": "9780134686097",
            "title": "Clean Code: A Handbook of Agile Software Craftsmanship",
            "author": "Robert C. Martin",
            "publisher": "Prentice Hall",
            "publication_year": 2008,
            "pages": 464,
            "genre": "Programming",
            "language": "English", 
            "price": 42.99,
            "rating": 4.7,
            "reviews_count": 2156,
            "format": "Paperback",
            "weight_kg": 0.68,
            "dimensions": "23.1 x 17.8 x 2.5 cm",
            "availability": "In Stock"
        },
        {
            "isbn": "9781491954243",
            "title": "Learning Python",
            "author": "Mark Lutz",
            "publisher": "O'Reilly Media",
            "publication_year": 2013,
            "pages": 1648,
            "genre": "Programming",
            "language": "English",
            "price": 67.99,
            "rating": 4.5,
            "reviews_count": 892,
            "format": "Paperback",
            "weight_kg": 2.1,
            "dimensions": "23.5 x 17.8 x 8.4 cm",
            "availability": "Limited Stock"
        },
        {
            "isbn": "9781617294136",
            "title": "Deep Learning",
            "author": "Ian Goodfellow, Yoshua Bengio, Aaron Courville",
            "publisher": "MIT Press",
            "publication_year": 2016,
            "pages": 800,
            "genre": "Machine Learning",
            "language": "English",
            "price": 89.99,
            "rating": 4.6,
            "reviews_count": 567,
            "format": "Hardcover",
            "weight_kg": 1.8,
            "dimensions": "25.4 x 20.3 x 5.1 cm",
            "availability": "In Stock"
        },
        {
            "isbn": "9780596517748",
            "title": "JavaScript: The Good Parts",
            "author": "Douglas Crockford",
            "publisher": "O'Reilly Media",
            "publication_year": 2008,
            "pages": 176,
            "genre": "Programming",
            "language": "English",
            "price": 29.99,
            "rating": 4.4,
            "reviews_count": 1834,
            "format": "Paperback",
            "weight_kg": 0.31,
            "dimensions": "17.8 x 12.7 x 1.3 cm",
            "availability": "In Stock"
        }
    ]
    return books

# Create all test files
def generate_test_files():
    # Get project root dynamically
    script_dir = Path(__file__).parent.absolute()
    base_path = script_dir.parent / "corpus"
    
    # Technical document
    with open(f"{base_path}/technical_documentation.txt", "w", encoding="utf-8") as f:
        f.write(create_technical_document())
    
    # Business report
    with open(f"{base_path}/business_report_q4_2024.txt", "w", encoding="utf-8") as f:
        f.write(create_business_report())
    
    # Research papers JSON
    with open(f"{base_path}/research_papers_collection.json", "w", encoding="utf-8") as f:
        json.dump(create_research_papers(), f, indent=2, ensure_ascii=False)
    
    # Company dataset CSV
    companies = create_company_dataset()
    with open(f"{base_path}/company_performance_2024.csv", "w", newline="", encoding="utf-8") as f:
        if companies:
            writer = csv.DictWriter(f, fieldnames=companies[0].keys())
            writer.writeheader()
            writer.writerows(companies)
    
    # Book catalog CSV
    books = create_book_catalog()
    with open(f"{base_path}/book_catalog_database.csv", "w", newline="", encoding="utf-8") as f:
        if books:
            writer = csv.DictWriter(f, fieldnames=books[0].keys())
            writer.writeheader()
            writer.writerows(books)
    
    print("✅ All test files generated successfully!")

if __name__ == "__main__":
    generate_test_files()

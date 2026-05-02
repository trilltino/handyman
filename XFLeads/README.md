# Tradesman Cold Call Scraper

A Python script that searches for local tradesmen/handymen using Google Places API and saves their contact information to an Excel spreadsheet for cold calling.

## Quick Start

### 1. Install Dependencies
```bash
pip install -r requirements.txt
```

### 2. Get a Google Places API Key
1. Go to [Google Cloud Console](https://console.cloud.google.com/)
2. Create a new project (or select existing)
3. Enable the **Places API** under "APIs & Services"
4. Create an API key under "Credentials"
5. Copy `.env.example` to `.env` and add your API key

### 3. Configure Your Search
Edit `.env` file:
```env
GOOGLE_PLACES_API_KEY=your_actual_api_key_here
SEARCH_LOCATION=Coventry, UK
SEARCH_RADIUS_METERS=16000
TRADE_TYPES=plumber,electrician,handyman,carpenter,roofer
```

### 4. Run the Script
```bash
python scraper.py
```

## Output

The script creates `XFTradesman Cold Call Document - SCRAPED.xlsx` with columns:
- Business Name
- Phone Number
- Address
- Website
- Rating
- Reviews
- Trade Type
- Status
- Scraped Date
- Notes (for your use)
- Called (tracking)

## Notes

- Only businesses with phone numbers are added
- Duplicates are automatically skipped
- Google Places API has a free tier (~$200/month credit)

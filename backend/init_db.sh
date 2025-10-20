#!/bin/bash

# Database initialization script for Handyman application

# Configuration
DB_NAME="handyman"
DB_USER="postgres"
DB_PASSWORD="Ab13cba46def79_"
DB_HOST="localhost"
DB_PORT="5432"

echo "Initializing Handyman database..."

# Create database if it doesn't exist
PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -tc "SELECT 1 FROM pg_database WHERE datname = '$DB_NAME'" | grep -q 1 || \
PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -c "CREATE DATABASE $DB_NAME"

# Run schema file
echo "Creating tables and schema..."
PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -f schema.sql

echo "Database initialization complete!"
echo "Connection string: postgresql://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME"

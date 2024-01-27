#!/bin/bash

# Exit script on any error
set -e

# Apply migrations to the development database
echo "Applying migrations to the development database..."
diesel migration run

# Set TEST_DATABASE_URL for the test database
export TEST_DATABASE_URL="postgres://test_user:test_password@localhost:5433/postgres_test"

# Apply migrations to the test database
echo "Applying migrations to the test database..."
diesel migration run --database-url $TEST_DATABASE_URL

echo "Migrations applied successfully."

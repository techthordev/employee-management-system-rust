DO $$
BEGIN
  IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = 'rustconnector') THEN
    CREATE USER rustconnector WITH PASSWORD 'rustconnector';
  END IF;
END $$;

ALTER DATABASE ems_db OWNER TO rustconnector;
GRANT ALL PRIVILEGES ON DATABASE ems_db TO rustconnector;
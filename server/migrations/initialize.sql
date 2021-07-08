CREATE TABLE IF NOT EXISTS blog_posts (
	created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMP WITH TIME ZONE,
	published_at TIMESTAMP WITH TIME ZONE,
	is_public BOOLEAN DEFAULT false,
	delta JSON,
	title VARCHAR(255) NOT NULL
);
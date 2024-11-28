pub const GET: &str = r#"server {
	listen 80;
	listen [::]:80;

	listen 443 ssl;
	listen [::]:443 ssl;
	# include snippets/snakeoil.conf;

	server_name _;

	location / {
		root /var/www/enchat/html;
    	index index.html;

		try_files $uri $uri/ =404;
	}
	location /api {
	    proxy_pass http://localhost:3989;
	}
}"#;
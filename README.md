# enchat

This is a project created for Ally Kotetsu's portfolio. It is **not** meant to be used, as it has not had its security reviewed, and is only meant to be a minimum viable product that demonstrates various programming concepts.

For anyone running this, run the setup.sh file, which builds each of the bins for release, runs them, and starts a systemd service to keep the backend running. Access on port 3989, and setup nginx to read /var/www/enchat/nginx as a virtual host directory.

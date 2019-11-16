docker run --rm -p 5434:5432 -e POSTGRES_PASSWORD=abc -e POSTGRES_USER=uppy -e POSTGRES_DB=up_db --name is_my_site_up -d postgres
echo "Waiting for db to run"
sleep 15
diesel migration run

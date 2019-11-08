docker run --rm -p 5434:5432 -e POSTGRES_PASSWORD=abc -e POSTGRES_USER=uppy -e POSTGRES_DB=up_db -d postgres
echo "Waiting for db to run"
sleep 15
diesel migration run

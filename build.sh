#!/bin/bash

# Loop through arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --run)
            run_flag=true # Set the flag
            ;;
        *)
            echo "Unknown option: $1"

            echo "Builds the application and copies it to the dist directory"
            echo "Provide the --run flag to run the app after building."
            exit 1
            ;;
    esac
    shift
done


# ----------- Build -----------------
echo "Building Frontend..."

cd ./frontend
npm run build

echo $'\nDone'

cd ..

echo "Copying new frontend..."

rm -rf ./dist
mkdir ./dist
cp -r ./frontend/dist ./dist/static

echo $'\nDone'


echo "Building backend..."

cd ./backend
cargo build --release

echo $'\nDone'

cd ..

echo "Copying new backend..."
cp -r ./backend/target/release/word-unscrambler ./dist/app
cp -r ./backend/data ./dist/data
echo $'\nDone'

# Execute a command if --run flag is provided
if [[ "$run_flag" == true ]]; then
  cd ./dist
  ./app
fi

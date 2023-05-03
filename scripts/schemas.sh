for dir in $PWD/contracts/*/; do
 cd $dir
 cargo run --example schema
 rm -rf schema/raw
 cd -
done

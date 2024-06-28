#/usr/bin/env -S bash 

cargo build --release 


for i in $(find target/release/  -maxdepth 1  -executable -type f -name 'rust*' ) ; do
    ln -sf $i ./$(basename $i)-release
done


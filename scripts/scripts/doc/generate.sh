cd $DIA_DIR;

cargo doc;
rm -r $DIA_DIR/docs;
mv $DIA_DIR/target/doc $DIA_DIR/docs;
echo "<meta http-equiv=refresh content=0;url=dia/index.html>" > $DIA_DIR/docs/index.html;

cd -;

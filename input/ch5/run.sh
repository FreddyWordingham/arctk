echo "Ch5: Healthy model"
mcrt output/ch5/mcrt/flesh input/ ch5/mcrt/flesh.json5;

echo "Ch5: Tumour model"
mcrt output/ch5/mcrt/tumour input/ ch5/mcrt/tumour.json5;

echo "Ch5: GNR infused model"
mcrt output/ch5/mcrt/tumour_plus_GNRs input/ ch5/mcrt/tumour_plus_GNRs.json5;

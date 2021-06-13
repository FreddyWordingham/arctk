babbage output/tmp input/ ch6/babbage/stripe.json5 ;
paste -d, ./output/tmp/stripe_000.csv \
    <(cut -d"," -f2- ./output/tmp/stripe_001.csv) \
    <(cut -d"," -f2- ./output/tmp/stripe_002.csv) \
    <(cut -d"," -f2- ./output/tmp/stripe_003.csv) \
    <(cut -d"," -f2- ./output/tmp/stripe_004.csv) \
    <(cut -d"," -f2- ./output/tmp/stripe_005.csv) \
    <(cut -d"," -f2- ./output/tmp/stripe_006.csv) \
    <(cut -d"," -f2- ./output/tmp/stripe_007.csv) \
    <(cut -d"," -f2- ./output/tmp/stripe_008.csv) \
    <(cut -d"," -f2- ./output/tmp/stripe_009.csv) \
    <(cut -d"," -f2- ./output/tmp/stripe_010.csv) \
    <(cut -d"," -f2- ./output/tmp/stripe_011.csv) \
    <(cut -d"," -f2- ./output/tmp/stripe_012.csv) \
    <(cut -d"," -f2- ./output/tmp/stripe_013.csv) \
    <(cut -d"," -f2- ./output/tmp/stripe_014.csv) \
    <(cut -d"," -f2- ./output/tmp/stripe_015.csv) \
    > stripes.csv

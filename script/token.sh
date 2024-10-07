http -f POST localhost:4000/token \
code_verifier=dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk \
code=$1 \
grant_type=authorization_code \
redirect_uri=http://localhost:3000/success

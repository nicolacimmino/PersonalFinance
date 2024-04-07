
# General

The OpenBanking service is responsible for pulling transaction data from bank institutions, using OpenBanking, through GoGardless.

At this stage there is no interface to create requisitions on GoCardless. Requisitions (bank connections) will need to be established once manually (using Postman). 
Once the requisition is created it will be enough to add the requisition ID to the `ob_accounts` table and the process will be able to sync.

# Setup

## Install and first run

You will need to:
* Create a GoCardless account and API key
* Create a Postgres empty database
* Install Rust
* Clone this repo and add a .env file in the `OpenBanking` folder with the following:

````
DATABASE_URL=postgresql://postgres:postgres@127.0.0.1:5432/pfinance

GOCARDLESS_SECRET_ID=[your GoCardless API Secret]
GOCARDLESS_SECRET_KEY=[your GoCardless API Secret Key]
GOCARDLESS_HOST=https://bankaccountdata.gocardless.com
````

* Run the app once with `cargo run`

If all is setup correctly this will run the `diesel` migrations and you should have a db schema with `ob_accounts` and `ob_transactions` tables.

## Create GoCardless requisitions

You will need to manually create requisitions with GoCardless as this service does not provide any facility to go through the OpenBanking authorization flow.

*NOTE:* you cannot use the GoCardless web interface to do this, requisitions created that way are not available through the API.





The OpenBanking service is responsible for pulling transaction data from bank institutions, using OpenBanking, through GoGardless.

At this stage there is no interface to create requisitions on GoCardless. Requisitions (bank connections) will need to be established once manually (using Postman). 
Once the requisition is created it will be enough to add the requisition ID to the `ob_accounts` table and the process will be able to sync.


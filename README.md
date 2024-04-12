# NourishNet3 - Food Aid Platform

## Overview
NourishNet3 is a revolutionary food aid platform leveraging blockchain technology to combat food insecurity. This platform connects donors, recipients, and food providers in a transparent and efficient manner.

### Version
- 0.1.0

### License
- MIT License

## Features
- Transparent food donation system
- Donor and FoodRequest types for efficient management
- Solana blockchain integration for decentralized processes
## demo video
link - https://youtu.be/u5WDtvHQisU

## Types
### Donor
- Manages donor accounts
- Tracks total and active donations, reward points, and processing fee

### FoodRequest
- Manages aid requests for recipients
- Tracks requested and received amounts, verification document URL, and approval status

## Methods
1. **create_donor**
   - Configures the donor account after contract deployment.

2. **add_donation**
   - Adds a new donation to the donor account.

3. **request_food_aid**
   - Allows recipients to request food aid.

4. **finalize_food_request**
   - Finalizes the food aid request and updates the FoodRequest account.

5. **update_donor_info**
   - Updates donor information, including reward points and active donations.

## Solana Integration
- Utilizes Solana blockchain for decentralized and traceable processes.
- Specific attributes for account initialization and mutation.

## How to Contribute
1. Fork the repository.
2. Implement new features or improvements.
3. Submit a pull request.

## Acknowledgments
- Inspired by the vision to revolutionize food aid through blockchain technology.
- Special thanks to the Codigo community for providing a robust platform.
## Contact
For inquiries or collaboration, contact me at j.vivekvamsi@gmail.com.

## How to Work Locally
Follow these steps to set up and work with NourishNet3 locally:

### Prerequisites
1. Must have codigo account

#### Clone the Repository

    git clone https://github.com/vivekjami/NourishNet3
    cd NourishNet3 
### In Codigo CLI
    Terminal -> New Terminal. In the new terminal, type the command "solana-test-validator"

    Finally, return to the terminal where you built the contract and type the command "solana program deploy target/deploy/nourishnet_3.so"
    It will give a program ID save* it some where. And paste it in the app.ts (progid).
### We can test this using
    app.ts in the program_client with  open a new terminal by going to Terminal -> New Terminal
    Navigate to the program client directory by typing the command "cd program_client", and inside the program_client directory, type the following command "npm install @types/node ts-node" after it completes, execute the command "npx ts-node app.ts".
    *Note: The solana-test-validator in the codigo some times does not work as it consumes more energy it will just resets the terminal or allocates another instance to the user  



# RestaurantAPI
A Rust REST API created using the Rocket framework with a SQLite database. Handles requests coming from clients (waiters) to handle orders coming in from customers at various tables. Below are the instructions for building, running, and then finally sending requests to the REST API.

**System Structure**

To briefly cover the structure of the system. This application, as mentioned above, is an API following the HTTP REST architecture. It can GET, PUT, POST, and DELETE from a database given commands (assumed to be coming in from Frontend elements). The API was created using the Rocket framework and interfaces with a SQLite database. 

The database stores the various orders for each table in the restaurant in the following manner. When customers are seated at a table, they are attended to by waitstaff. The waitstaff takes the customer's orders and pushes it to the database. For eg. If the waiter is serving some table labelled by the restaurant as **Table T** then when the waitstaff sends a POST request to the database for **Table T**, the API creates a SQLite table in the database called **table_T**. If the table already exists, then it simply updates the pre-existing orders contained in the table. The behaviour of the endpoints will be elaborated further upon. 

Regarding the menu items being inserted to and queried from the database, Rust is a bit different to traditional OOP based languages and so I have made the syntax generic to any language background.

       public class Item {
           int item_id;   // Primary id for each menu item.
           int quantity;  // Quantity of the menu item ordered for the specific table
           int prep_time; // Preparation time (minutes) required to cook the item. (random number between 5-15 minutes)
       }

Every table can at most have 1 of any item_id.

tl;dr - System contains 1 database with multiple SQLite tables corresponding to the physical tables in the restaurant with naming convention **table_<table_num>**. Every menu item has a unique item_id field, quantity field, and preparation time field (a random number between 5-15). 

**1. How To Build + Run The Application:**

1.1 Clone the repository into your computer. This can be done by downloading the .zip file from GitHub or using the 'git clone' command.
 
1.2 In your command line terminal, navigate to the repository directory in your local machine

1.3 In order to run the a Rust executable, you require the Rustup toolchain. Below is the bash command to install Rustup for Unix-based OS's.
   
       curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   For installation on Windows, go to https://www.rust-lang.org/tools/install and follow the steps to setup Rustup.

1.4 Once Rustup is installed, you need the Nightly version of the Rust build tools in order to run this application. You can do this by typing in the following in your command terminal.

       rustup default nightly
       
Please note that you need Rustup to do so.

1.5 Once these tools are installed, you are ready to run the program. Rocket applications can be run in 3 modes; dev, staging, and production modes. The thread count for the dev and production modes have been set to 10 (this can be altered in Rocket.toml). By default, Rocket apps are run in dev mode. To run the app in dev mode, simply type the following in the command line.
  
       cargo run
       
1.6 This will run the app in dev mode. The compiler does not optimize the binary executable when run in dev mode and thus to run the production version of this app, the following command must be used.

       cargo run --release
       
 The Rust compiler now optimizes the binary executable thus resulting in better runtime and performance.
   
 **2. Sending HTTP requests to Server:**
   
 Now that the REST API is running, we can send various requests to it. Before you do that, please have a look at the endpoints provided by the API, the parameters you can send through as well as the expected return type and value. I have also provided templates for the HTTP requests that you can send the REST API with examples.
 
2.1 ***index: GET***

Parameters:

       None
       
Returns: String splice

       Welcome! 

Request Syntax for index( ):

       curl localhost:8000/
       
Example Request: Accessing localhost (127.0.0.1) on port 8000 which prints the following.

       curl localhost:8000/
       
2.2 ***add_item: POST***

Parameters:

       item_id, quantity, table_num
       
Returns: String splice

       Status Message (displayed in JSON) 

Request Syntax for index( ):

       curl localhost:8000/item_id/quantity/table_num
       
Example Request: Inserting into table table_num a menu item with a unique identifier item_id and with quantity set to quantity.

       curl localhost:8000/

       
 

   



**Assumptions & Notes:**

1. The Rocket framework is multithreaded intrinsically; each time a client communicates to the HTTP server, Rocket assigns that client-server connection a thread. This allows concurrent processing and speeds up general operation. Rocket also allows developers to hardcode the maximum number of threads that Rocket can allocate to various client-server connections. This mitigates a potential DDoS attack by any bad actors sending in multiple HTTP requests which could strain the resources on an unprotected server. By default, Rocket allocates N threads where N = number of CPU cores. For this project, both development and product environments were setup to accomodate 10 workers (i.e. threads). The assumption here is that in order to take full advantage of Rocket's multi-threaded nature, the server computer must have the necessary computational resources.

2. The endpoint responsible for inserting menu items into the database is only capable of inserting N of a particular item at a time per each request. In a case where the waiter would need to input M different menu items, they would need to send M different HTTP PUT requests to the server. This ofcourse is not practical, it would be much better to store various Item structs into a dynamic data structure such as a Vector and when POSTing to the server, just iterate through the Vector and update the database. However, as this project only includes design of an API rather than an associate FrontEnd element, this feature was not implemented. In a more practical scenario, the waiter would store the orders for a table on some frontend element and then that data could then be parsed and sent to the API for Database storage.

5. Preparation time assigned to items were static and procured using a Random number generator function. I assumed that if an item was added with a quantity exceeding 1, its preparation would still stay the same as the items are assumed to be prepared in parallel. For eg. Suppose item_id = 1 corresponds to a Margherita pizza. If two pizzas are ordered and each of them have a prep_time of 10mins, then the total prep_time of the two pizzas would still remain 10mins as the assumption is that the restaurant has the necessary tools to prepare both pizzas at the same time. Another prep_time assumption that I made was that a Margherita pizza ordered at some time would not have the same prep_time as a Margherita pizza ordered in another time. I assumed that prep_times for the same item would vary throughout the day in a real life scenario. There are many factors that influence the potential prep_time of a menu item. Factors such as restaurant congestion, available chefs, how many of a particular item are being prepared, kitchen facilities available are just some that I can mention that would influence preptime and to handle these factors was beyond the scope of this project. 


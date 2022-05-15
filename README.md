# RestaurantAPI
A Rust API created using the Rocket framework with a SQLite database. Handles requests coming from clients (waiters) to handle orders coming in from customers at various tables.

**Structure of API**

**How to run the application:**

**API Routes:**

**Assumptions & Notes:**

1. The Rocket framework is multithreaded intrinsically; each time a client communicates to the HTTP server, Rocket assigns that client-server connection a thread. This allows concurrent processing and speeds up general operation. Rocket also allows developers to hardcode the maximum number of threads that Rocket can allocate to various client-server connections. This mitigates a potential DDoS attack by any bad actors sending in multiple HTTP requests which could strain the resources on an unprotected server. By default, Rocket allocates N threads where N = number of CPU cores. For this project, both development and product environments were setup to accomodate 10 workers (i.e. threads).

2. The endpoint responsible for inserting menu items into the database is only capable of inserting N of a particular item at a time per each request. In a case where the waiter would need to input M different menu items, they would need to send M different HTTP PUT requests to the server. This ofcourse is not practical, it would be much better to store various Item structs into a dynamic data structure such as a Vector and when POSTing to the server, just iterate through the Vector and update the database. However, as this project only includes design of an API rather than an associate FrontEnd element, this feature was not implemented. In a more practical scenario, the waiter would store the orders for a table on some frontend element and then that data could then be parsed and sent to the API for Database storage.

5. Preparation time assigned to items were static and procured using a Random number generator function. I assumed that if an item was added with a quantity exceeding 1, its preparation would still stay the same as the items are assumed to be prepared in parallel. For eg. Suppose item_id = 1 corresponds to a Margherita pizza. If two pizzas are ordered and each of them have a prep_time of 10mins, then the total prep_time of the two pizzas would still remain 10mins as the assumption is that the restaurant has the necessary tools to prepare both pizzas at the same time. Another prep_time assumption that I made was that a Margherita pizza ordered at some time would not have the same prep_time as a Margherita pizza ordered in another time. I assumed that prep_times for the same item would vary throughout the day in a real life scenario. There are many factors that influence the potential prep_time of a menu item. Factors such as restaurant congestion, available chefs, how many of a particular item are being prepared, kitchen facilities available are just some that I can mention that would influence preptime and to handle these factors was beyond the scope of this project. 


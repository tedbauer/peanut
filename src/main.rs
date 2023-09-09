#[macro_use] extern crate rocket;

use rocket::http::ContentType;

#[get("/")]
fn index() -> (ContentType, &'static str) {
    (ContentType::HTML, r#"
    <!DOCTYPE html>
    <html>
   <head>
   <style>
   .box {
       border-style: solid;
       padding: 25px;
       width: 750px;
       box-shadow: 5px 5px 0 0 black;
       margin-top: 100px;
       margin-left: 100px;
   }
   .menu {
       border-style: solid;
       padding: 25px;
       width: 300px;
       box-shadow: 5px 5px 0 0 black;
       margin-left: 100px;
       margin-top: 20px;
       font-size: 25px;
   }
   </style>
   </head>
   <body>
   <title>hello</title>
   <div class="box">
       <h1>Example card
   
           <span style="float:right; font-size: 20px"><a href="google.com">edit</a></span>
   
       </h1>
       <h2 style="color: grey">09/09/2023</h2>
       <p>
       Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
       </p>
       <h3 style="color: grey">References</h3>
       <ol>
           <li>Reference 1</li>
           <li>Reference 2</li>
           <li>Reference 3</li>
       </ol>
   
   </div>
   
   <div class="menu">
       <a href="google.com">+ Write a new card</a>
   </div>
   </body>
    </html>
    "#)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
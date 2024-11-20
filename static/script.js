document.addEventListener("DOMContentLoaded", function() {
    const button = document.getElementById("myButton");
    const message = document.getElementById("message");
    const contactForm = document.getElementById("contactForm");

    console.log("JavaScript is running");

    button.addEventListener("click", function() {
        message.textContent = "Button clicked!";
        console.log("Button was clicked");
    });

    contactForm.addEventListener("submit", function(event) {
        event.preventDefault();
        const name = document.getElementById("name").value;
        const email = document.getElementById("email").value;
        const messageInput = document.getElementById("messageInput").value;

        console.log(`Form submitted with Name: ${name}, Email: ${email}, Message: ${messageInput}`);
        alert("Form submitted successfully!");

        contactForm.reset();
    });
});
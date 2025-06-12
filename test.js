function myFunction() {
    var x = document.getElementById("myTopnav");
    if (x.className === "topnav") {
        x.className += " responsive";
    } else {
        x.className = "topnav";
    }
}
// Select the <a> tag (you can select by id or any other way)
var aTag = document.getElementById('close');

// If it doesn't exist yet, create it and append to the body (optional)
if (!aTag) {
    aTag = document.createElement('a');
    aTag.id = 'close';
    document.body.appendChild(aTag);
}

// Set all important values
aTag.href = "javascript:void(0);";
aTag.className = "icon";
aTag.setAttribute("onclick", "myFunction()");

// Optionally set the inner HTML (icon)
aTag.innerHTML = '<i class="fa fa-bars"></i>';
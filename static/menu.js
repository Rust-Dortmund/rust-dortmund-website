function myFunction() {
    var x = document.getElementById("myTopnav");
    if (x.className.includes("responsive")) {
        x.className = x.className.replace(" responsive", "");
    } else {
        x.className += " responsive";
    }
}
function waitForElement() {
    const el = document.getElementById('close');
    if (el) {
        console.log('#close is in the DOM!');
        var aTag = document.getElementById('close');
        aTag.href = "javascript:void(0);";
        aTag.setAttribute("onclick", "myFunction()");
        // ... your logic here ...
    } else {
        setTimeout(waitForElement, 100);
    }
}
waitForElement();
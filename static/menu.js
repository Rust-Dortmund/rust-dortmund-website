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
        var links = document.querySelectorAll('#myTopnav a');
        links.forEach(function(link) {
            if (link.className.includes("icon")) {
                return
            }
            link.addEventListener('click', function() {
                var parent = this.parentElement;
                if (parent.className.includes("responsive")) {
                    parent.className = parent.className.replace(" responsive", "");
                }
            });
        });
        var aTag = document.getElementById('close');
        aTag.href = "javascript:void(0);";
        aTag.setAttribute("onclick", "myFunction()");
        console.log("Element found and onclick event set.");
        //for every child that is a link, add an onclick event that removes the class "responsive" from the parent element
        // ... your logic here ...
    } else {
        setTimeout(waitForElement, 100);
    }
}
waitForElement();


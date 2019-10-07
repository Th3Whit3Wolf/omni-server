document.addEventListener('DOMContentLoaded', () => {
    // Get all "navbar-burger" elements
    const $navbarBurgers = Array.prototype.slice.call(document.querySelectorAll('.navbar-burger'), 0);
    // Check if there are any navbar burgers
    if ($navbarBurgers.length > 0) {
        // Add a click event on each of them
        $navbarBurgers.forEach(el => {
            el.addEventListener('click', () => {
                // Get the target from the "data-target" attribute
                const target = el.dataset.target;
                const $target = document.getElementById(target);
                // Toggle the "is-active" class on both the "navbar-burger" and the "navbar-menu"
                el.classList.toggle('is-active');
                $target.classList.toggle('is-active');
            }
            );
        }
        );
    }
}
);

/*
document.getElementById("searchBtn").onclick = function() {
    var link = document.getElementById("searchBtn");
    link.setAttribute("href", "the new url");
};
*/

// dropdown menu
document.addEventListener('DOMContentLoaded', function () {

    // Dropdowns
    var $selects = getAll('.select');
    if ($selects.length > 0) {
        $selects.forEach(function ($el) {
            $el.addEventListener('click', function (event) {
                event.stopPropagation();
                //$el.classList.toggle('is-active');
                const selection = ['kernel', 'arch', 'release', 'repo'];
                selection.forEach(showSelected);
            });
        });
    }

    // Functions
    function getAll(selector) {
        return Array.prototype.slice.call(document.querySelectorAll(selector), 0);
    }

    function showSelected(category) {
        var selector = document.getElementById(category);
        var val = selector[selector.selectedIndex].value;
        console.log(val);
    }
});

// Search for packages
function searchBar() {
    // Declare variables
    var input, filter, table, td, i, txtValue;
    input = document.getElementById("Search");
    filter = input.value.toUpperCase();
    table = document.getElementById("PackageList");
    tr = table.getElementsByTagName("tr");

    // Loop through all table rows, and hide those who don't match the search query
    for (i = 0; i < tr.length; i++) {
        td = tr[i].getElementsByTagName("td")[4];
        if (td) {
            txtValue = td.textContent || td.innerText;
            if (txtValue.toUpperCase().indexOf(filter) > -1) {
                tr[i].style.display = "";
            } else {
                tr[i].style.display = "none";
            }
        }
    }
}

var rows = document.getElementById('PackageList').getElementsByTagName("tbody")[0].getElementsByTagName("tr").length;
document.getElementById('packages').innerHTML = '4';
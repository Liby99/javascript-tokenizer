// Open Weather API Key: 89635bfae26331da7c18c8fa78db7fac

// =========================================================================================
// EVENT LISTENER FOR ON-CLICK OF CITY-BUTTON
// =========================================================================================
$(document).on("click", ".city-button", function () {

    // console.log($(this).attr("city"));

    // =========================================================================================
    // OPEN-WEATHER API AJAX CALL
    // =========================================================================================

    var city = $(this).attr("city");
    var APIkey = "89635bfae26331da7c18c8fa78db7fac";
    var queryURL = "https://api.openweathermap.org/data/2.5/weather?q=" + city + "&appid=" + APIkey;

    $.ajax({
      url: queryURL,
      method: "GET"
    }).then(function (response) {

      //console.log(response);

      var weatherText = $("#weather-text");

// PROTECT_ICLR_START
// weatherText.html
// PROTECT_ICLR_END
// ("<h5>" + "Weather in " + city + "</h5>");

      // SHOW CITY
        // console.log(response.name);
        // var cityHolder = $("#city-holder");
        // cityHolder.text("City: " + response.name)

      // SHOW TEMPERATURE
        //console.log(response.main.temp);
        var tempF = ((response.main.temp - 273.15) * 1.80 + 32).toFixed(0);
        //console.log(tempF)
        var tempHolder = $("#temp-holder")
        tempHolder.html("Temp: " + tempF + "&#8457;")


      // SHOW WEATHER DESCRIPTION
        // console.log(response.weather[i].description);
        var weatherHolder = $("#weather-holder");
        var description = response.weather[0].description;
        weatherHolder.text(description);


      // SHOW WEATHER ICON
        $.getJSON(queryURL, function (data) {
        // console.log(data);
        // console.log(response.weather[0].icon);

        var icon = response.weather[0].icon;
        var url = 'https://openweathermap.org/img/w/' + icon + '.png'
        // console.log(url);

        var iconHolder = $("#icon-holder")
            iconHolder.html("<img class = 'newIcon' src=" + url + ">");

      }); // CLOSE GET JSON CALL FOR WEATHER ICON

    }); // CLOSE WEATHER-API JSON CALL

  }); // CLOSE EVENT LISTENER FOR ON-CLICK FOR CLICK-BUTTON
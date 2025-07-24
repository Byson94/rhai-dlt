fn seperator() {
    return box(#{
        "class": "separator",
        "width": "1px",
        "height": "20px",
        "background_color": "#888",
        "margin": "0 8px"
    })
}

fn widget1() {
    return box(#{
        "class": "widget1",
        "orientation": "h",
        "space_evenly": false,
        "halign": "start",
        "spacing": 5
    }, [
        button(print("Hello there!"), "greet") // param1 = onclick. param2 = initvalue.

        seperator()

        button(print("Bye bye!"), "say bye")
    ])
}

fn bar() {
    return centerbox({orientation: "h"}, [
        widget1()
    ])
}

fn name() {
    // do same here
}

enter([
    poll("varname", "1s", "./script.sh", "initialVal"),

    defwidget(bar()),
    defwindow("name", #{
        "monitor": 0,
        "windowtype": "dock",
        "geometry": {
            // geometry stuff here
        }
        "reserve": {
            // reserve stuff here
        }
    }, name()),
])
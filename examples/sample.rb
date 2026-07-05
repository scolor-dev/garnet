page do,
    background: "#f8fafc",
    padding: 24
    column do,
        background: "#ffffff",
        padding: 20,
        margin: 12
        text "Garnet",
            color: "#3b82f6",
            size: 28

        input "Type something...",
            width: 320,
            height: 36,
            margin: 8

        input placeholder: "Name",
            width: 320,
            height: 36,
            margin: 8

        image "examples/assets/logo.png",
            width: 128,
            margin: 8

        text "Ruby Web Protocol",
            color: "#334155",
            size: 18,
            margin: 8

        row do,
            background: "#eef2ff",
            padding: 8,
            margin: 8
            button "Open",
                width: 160,
                height: 40

            button "Exit",
                width: 120,
                height: 40
        end

        text "Hidden text",
            visible: false
    end
end

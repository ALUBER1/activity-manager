export function change_background(input) {
    const regex = /(\w+-\w+)(\#\w+)/i;
    const item = input.match(regex)[1];
    const value = input.match(regex)[2];
    switch(item) {
        case "background-color":
            document.documentElement.style.setProperty('--background-color', value);
            document.getElementById("btn").style.setProperty("background-color", value);
            document.getElementsByClassName("pcr-result")[0].value = value;
            break;
        case "title-color":
            document.documentElement.style.setProperty('--head-background-color', value);
            break;
    }
}
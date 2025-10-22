export function change_background(input) {
    console.log(input)
    const regex = /(.+)(\#\w+)\#(\d)/i;
    const item = input.match(regex)[1];
    const value = input.match(regex)[2];
    const index = input.match(regex)[3];
    console.log(`item: ${item}, value: ${value}`)
    document.documentElement.style.setProperty(`--${item}`, value);
    document.getElementById(item).style.setProperty("background-color", value);
    document.getElementsByClassName("pcr-result")[index].value = value;
}
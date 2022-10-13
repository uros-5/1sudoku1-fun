export function keyboardMap(): Map<string, number> {
  const map = new Map();
  for (let i = 0; i < 10; i++) {
    map.set(String(i), i);
  }
  let arrows = ["ArrowLeft", "ArrowUp", "ArrowRight", "ArrowDown"];
  let value = 11;
  for (let i = 0; i < 4; i++) {
    map.set(String(arrows[i]), value);
    value += 1;
  }
  map.set("Escape", 15);
  return map;
}

# pinyin-hanzi-wasm
A pinyin to hanzi WASM npm package that returns the most likely Chinese characters and words based on a frequency corpus

# 🉐 What is it?

A package to convert pinyin to hanzi (Chinese characters). 🇨🇳 For example, "nihao" ➡️ "你好"

# 🚀 How it works

1. 📚 Using a large Chinese corpus, scan the most frequently occurring words.
2. 📊 Order the words based on frequency.
3. 🧐 When the user types in pinyin, find the most common characters.
4. 🔄 If no matching values are found, return the top partial matching values.
5. ⚡ Uses WebAssembly (WASM) for blazing-fast performance.

# 📝 How to use it

```javascript
import init, { pinyin_to_hanzi } from "./pinyin_to_hanzi.js";

async function initializeWASM() {
	await init(); // Initialize the WASM module
}

async function runTest(pinyinInput) {
	const result = pinyin_to_hanzi(pinyinInput);
	// Result is a list of strings
}

initializeWASM(); // Initialize WASM as soon as the script loads
```

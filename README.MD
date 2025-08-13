<h1 align="center">rolador de dados rpg by wer</h1>

<p align="center">
  <a href="https://www.rust-lang.org/">
    <img src="https://img.shields.io/badge/Rust-1.70+-orange?logo=rust&logoColor=white" alt="Rust">
  </a>
  <a href="https://github.com/WerG0D/rpg-dice-sim/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-green" alt="License">
  </a>
  <img src="https://img.shields.io/github/stars/WerG0D/rpg-dice-sim?style=social" alt="Stars">
  <img src="https://img.shields.io/github/last-commit/WerG0D/rpg-dice-sim?color=blue" alt="Last Commit">
</p>

---
projetinho simples pra rolar dados em rust, feito com intuito de aprender a linguagem e pra uso pessoal

usos como: 
- `2d6+3`
- `3d6+2d8-1`
- `d20+5` com **vantagem** (`--adv`) ou **desvantagem** (`--dis`)

---

## Feats

múltiplos termos na mesma expressão (`3d6+2d8-1`)  
vantagem e desvantagem em `d20`  
estatísticas (mínimo, máximo, média) com `--times`  
modor silencioso (`-q`) para exibir apenas o total  

---

## 🚀 Como Usar

### Uma rolagem simples
rpg-dice-sim "2d6+3"

### Múltiplas rolagens com estatísticas
rpg-dice-sim "3d6+2d8-1" --times 10

### Vantagem em d20
rpg-dice-sim "d20+5" --adv --times 5

### Desvantagem em d20
rpg-dice-sim "d20+5" --dis

### Somente total, sem detalhes
rpg-dice-sim "4d6-2" -q

## 📦 Instalação
```bash
cargo install --path .




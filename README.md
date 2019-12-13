# НИИЭТ К1921ВК01Т

## Сборка

```bash
# Установка необходимых утилит
cargo install svd2rust
cargo install form
pip3 install --upgrade --user svdtools

# Собственно, сборка
svd patch ./k1921vk01t.yaml && \
svd2rust -i ./k1921vk01t.svd.patched && \
rm -rf ./src/ && \
form -i ./lib.rs -o ./src/ && \
rm ./lib.rs && \
cargo fmt
```

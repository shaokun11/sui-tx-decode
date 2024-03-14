import { execa } from "execa";
// await execa("cargo build", []);
let tx_data =
    "AAACACBqEB6aOvXIBwES+Ahkizbvv43uihqC3kbZUE6WoRCKFwEAjvdvVsOZYzousxC8qRJOXy84znOeqsu2YAaIgE4HhEgCAAAAAAAAACB9w3+ufZMpihJFwxtCBojBaGy00TVtFxgN2C6TpIPFqwEBAQEBAAEAAAS0l6kWtGVmCaf6gnoJGE1vR2gdO6dM4NejbGSysfiHAZ+Q9/hmzCnfsdpjc86U+dldylpA9OF2mRjuv5+64AvTAgAAAAAAAAAgjleHL0UiRGjh/BfIFHCJ3EMY/dQA22c2TvNQyVJnbYUEtJepFrRlZgmn+oJ6CRhNb0doHTunTODXo2xksrH4hwoAAAAAAAAAoIYBAAAAAAAA";
tx_data =
    "AAABAQGJeL4ADwYhIlia2yYMZ+dSd+iNzGC3JqO7RCekGocNbFInDgAAAAAAAQEAcfU1+QF4q9Nvbp3PhpyShGy17zxMK54Vn6eUmqHxvl0HY291bnRlcglpbmNyZW1lbnQAAQEAAGQpV670B1MRv4/oPaQKGstSexun6H/lVkfC68ZQA877AGQpV670B1MRv4/oPaQKGstSexun6H/lVkfC68ZQA8776AMAAAAAAAAAdDukCwAAAAA=";
let signature =
    "AKD4XdltkCyBi1Heb4EJJ3lzuV3F4u7+CYeaE+Fd7qXpaT17yd4tHWjMf4CWq3TuXBLxTpkc2MV39P6p7eMV8QnqvbuA0Q1Bqu4RHV3JPpqmH+C527hWJGUBOZN1j9sg8w==";

const { stdout } = await execa("./target/debug/sui-transaction-decode.exe", [tx_data, signature]).catch(
    () => -1
);
if (stdout) {
    console.log(JSON.parse(stdout));
} else {
    console.log("error");
}

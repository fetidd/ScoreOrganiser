export default function SnackBar({ msg, color }: Props) {
  let snackbar = document.createElement("div");
  snackbar.className = "snackbar";
  snackbar.classList.add(color);
  snackbar.textContent = msg;
  document.body.appendChild(snackbar);
  snackbar.classList.add("show");
  setTimeout(() => {
    snackbar.classList.remove("show");
    document.body.removeChild(snackbar);
  }, 3000);
}

type Props = {
  msg: string,
  color: string,
}

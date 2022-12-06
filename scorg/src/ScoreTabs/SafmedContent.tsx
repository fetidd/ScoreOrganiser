import { useEffect } from "react"

export default function SafmedContent({ plot }: Props) {

  useEffect(() => {
    document.querySelector("#safmed-chart")!.innerHTML = plot
  }, [plot])
  
  return (
    <div id="safmed-content">
      <div id="safmed-chart" className="chart" ></div>
    </div>
  )
}

interface Props {
  plot: string,
}

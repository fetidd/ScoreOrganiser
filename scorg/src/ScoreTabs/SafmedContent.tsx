import { useEffect, useState } from "react"

export default function SafmedContent({ getPlot, id }: Props) {
  const [plot, setPlot] = useState(<span>Fetching plot...</span>)

  useEffect(() => setPlot(getPlot()), [id])

  return (
    <div id="safmed-content" className="content">
      <div className="chart">{plot}</div>
    </div>
  )
}

interface Props {
  getPlot: Function,
  id: string
}

import { useEffect } from "react"
import { Scatter } from 'react-chartjs-2';
import type { ChartOptions, ChartData, ScatterDataPoint} from "chart.js"

export default function SafmedContent({}) {

  let data: ChartData<"scatter"> = {
    datasets: [
      {
        label: "correct",
        data: [
          {x: 78, y: 10},
        ] as ScatterDataPoint[]
      },
    ]
  }

  let options: ChartOptions<"scatter"> = {}
  
  return (
    <div id="safmed-content">
      <Scatter options={options} data={data} />
    </div>
  )
}


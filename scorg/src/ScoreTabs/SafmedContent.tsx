import { ChartData, Chart, registerables, ChartOptions, LogarithmicScale } from "chart.js";
import { useEffect, useState } from "react"
import { Scatter } from 'react-chartjs-2';
import 'chartjs-adapter-moment';
import moment from "moment"
import { Score } from "./Score";

Chart.register(...registerables)
Chart.defaults.elements.point.radius = 6
Chart.defaults.elements.point.hoverRadius = 8

function getDate(date: Date) {
  return Number.parseInt(moment(date).format("x"))
}

export default function SafmedContent({scores}: Props) {
  const [correctData, setCorrectData] = useState([] as Point[])
  const [incorrectData, setIncorrectData] = useState([] as Point[])
  
  useEffect(() => {
    let points = scores.map(scoreToDataPoints)
    setCorrectData(points.map(p => p.correct))
    setIncorrectData(points.map(p => p.incorrect))
  }, [scores])

  const data: ChartData<"scatter"> = {
    datasets: [
      {
        label: 'Correct',
        data: correctData,
        backgroundColor: 'rgb(99, 255, 132)'
      },
      {
        label: 'Not yet',
        data: incorrectData,
        backgroundColor: 'rgb(255, 99, 132)'
      },
    ],
  };


  const options: ChartOptions<"scatter"> = {
    scales: {
      y: {
        type: 'logarithmic',
        position: 'left',
        display: false
      },
      x: {
        type: "time",
        display: false,
      }
    }
  }

  return (
    <div id="safmed-content">
      <Scatter options={options} data={data} />
    </div>
  )
}

interface Props {
  scores: Score[]
}


function scoreToDataPoints(score: Score) {
  const date = getDate(score.date)
  return {
    correct: {
      x: date,
      y: score.correct
    },
    incorrect: {
      x: date,
      y: score.incorrect
    }
  }
}

type Point = {
    x: number,
    y: number
}

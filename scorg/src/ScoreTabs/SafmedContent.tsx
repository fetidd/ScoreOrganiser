export default function SafmedContent({ id }: Props) {
  return (
    <div id="safmed-content">
      <div className="chart"></div>
    </div>
  )
}

interface Props {
  id: string
}

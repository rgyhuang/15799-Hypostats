// tick length
const TICK_LENGTH = 6;

export default function AxisLeft({ yScale, yValues }) {
  const range = yScale.range();
  const numberOfTicksTarget = 1;

  return (
    <>
      {/* Main vertical line */}
      <path
        d={["M", 0, range[0], "L", 0, range[1]].join(" ")}
        fill="none"
        stroke="currentColor"
      />

      {/* Ticks and labels */}
      {yScale.ticks(numberOfTicksTarget).map((value, i) => (
        <g key={value} transform={`translate(0, ${yScale(value)})`}>
          <line x2={-TICK_LENGTH} stroke="currentColor" />
          <text
            key={value}
            style={{
              fontSize: "12px",
              textAnchor: "middle",
              fill: "white",
              transform: "translateX(-20px)",
            }}
          >
            {yValues[i]}
          </text>
        </g>
      ))}
    </>
  );
}

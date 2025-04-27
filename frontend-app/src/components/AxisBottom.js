import { useMemo } from "react";

// tick length
const TICK_LENGTH = 6;

export default function AxisBottom({ xScale, bounds }) {
  const range = xScale.range();

  const ticks = useMemo(() => {
    const numberOfTicksTarget = bounds.length - 1;
    return xScale.ticks(numberOfTicksTarget).map((value) => ({
      value,
      xOffset: xScale(value),
    }));
  }, [xScale]);

  return (
    <>
      {/* Main horizontal line */}
      <path
        d={["M", range[0], 0, "L", range[1], 0].join(" ")}
        fill="none"
        stroke="currentColor"
      />

      {/* Ticks and labels */}
      {ticks.map(({ value, xOffset }) => (
        <g key={value} transform={`translate(${xOffset}, 0)`}>
          <line y2={TICK_LENGTH} stroke="currentColor" />
          <text
            key={value}
            style={{
              fontSize: "12px",
              fill: "white",
              textAnchor: "middle",
              transform: "translateY(20px)",
            }}
          >
            {JSON.stringify(bounds[value])}
          </text>
        </g>
      ))}
    </>
  );
}

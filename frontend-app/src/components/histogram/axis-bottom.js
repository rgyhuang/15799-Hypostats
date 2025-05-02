import { useMemo } from "react";

// tick length
const TICK_LENGTH = 6;

export default function AxisBottom({ xScale, bounds, small }) {
  const range = xScale.range();

  const ticks = useMemo(() => {
    const numberOfTicksTarget = bounds.length - 1;
    return xScale.ticks(numberOfTicksTarget).map((value) => ({
      value,
      xOffset: xScale(value),
    }));
  }, [xScale, bounds.length]);

  return (
    <>
      {/* Main horizontal line */}
      <path
        d={["M", range[0], 0, "L", range[1], 0].join(" ")}
        fill="none"
        stroke="currentColor"
      />

      {/* Ticks and labels */}
      {ticks.map(({ value, xOffset }, idx) => {
        let translate =
          !small && idx % 2 === 0 ? "translateY(40px)" : "translateY(20px)";
        let tickLabel =
          small && ticks.length > 10 ? "" : JSON.stringify(bounds[value]);
        if (!small && tickLabel.length > 5) {
          tickLabel = tickLabel.slice(0, 5) + "...";
        }
        return (
          <g key={value} transform={`translate(${xOffset}, 0)`}>
            <line y2={TICK_LENGTH} stroke="currentColor" />
            <text
              key={value}
              style={{
                fontSize: "12px",
                fill: "white",
                textAnchor: "middle",
                transform: translate,
              }}
            >
              {tickLabel}
            </text>
          </g>
        );
      })}
    </>
  );
}

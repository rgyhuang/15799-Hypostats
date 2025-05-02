import * as d3 from "d3";
import AxisBottom from "./axis-bottom";
import AxisLeft from "./axis-left";

const MARGIN = { top: 30, right: 30, bottom: 50, left: 50 };

export default function Histogram({
  width,
  height,
  xData,
  yData,
  yValue,
  small,
}) {
  // Layout. The div size is set by the given props.
  // The bounds (=area inside the axis) is calculated by substracting the margins
  const boundsWidth = width - MARGIN.right - MARGIN.left;
  const boundsHeight = height - MARGIN.top - MARGIN.bottom;
  const BUCKET_PADDING = 0;

  // Compute the scales (usually done using the dataset as input)
  const xScale = d3
    .scaleLinear()
    .domain([0, xData.length - 1])
    .range([0, boundsWidth]);
  const yScale = d3.scaleLinear().domain([0, 1]).range([boundsHeight, 0]);
  const yValues = [0, yValue.toFixed(2)];

  const bucketWidth = boundsWidth / (xData.length - 1);
  let allRects = [];
  for (let i = 0; i < xData.length - 1; i++) {
    const scale = yScale(yData[i]);
    const barHeight = boundsHeight - scale;
    const barY = scale;

    allRects.push(
      <rect
        key={i}
        fill="#69b3a2"
        stroke="black"
        x={i * bucketWidth + BUCKET_PADDING / 2}
        width={bucketWidth - BUCKET_PADDING}
        y={barY}
        height={barHeight}
      />
    );
  }

  return (
    <div style={{ overflowX: "auto" }}>
      <svg width={width} height={height} shapeRendering={"crispEdges"}>
        <g
          width={boundsWidth}
          height={boundsHeight}
          transform={`translate(${[MARGIN.left, MARGIN.top].join(",")})`}
          overflow={"visible"}
        >
          {/* graph content */}
          {allRects}

          {/* Y axis */}
          <AxisLeft yScale={yScale} yValues={yValues} />

          {/* X axis, use an additional translation to appear at the bottom */}
          <g transform={`translate(0, ${boundsHeight})`}>
            <AxisBottom xScale={xScale} bounds={xData} small={small} />
          </g>
          <text
            transform={`translate(${boundsWidth / 2 - 100}, ${
              boundsHeight + 25
            })`}
            style={{ fill: "white", fontSize: "14px" }}
          >
            {small && xData.length > 10 ? "Too many ticks to show bounds" : ""}
          </text>
        </g>
      </svg>
    </div>
  );
}

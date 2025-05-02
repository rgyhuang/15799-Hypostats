import Table from "react-bootstrap/Table";
import HistogramWrapper from "./histogram/histogram-wrapper";

function sanitizeStats(
  stats,
  histoModalState,
  showHistoModal,
  hideHistoModal,
  idx
) {
  const cleanStats = new Map();
  cleanStats.set("Null Fraction", [stats["stanullfrac"], "stanullfrac"]);
  cleanStats.set("Average Entry Width (bytes)", [
    stats["stawidth"],
    "stawidth",
  ]);
  cleanStats.set("Distinct Elements", [stats["stadistinct"], "stadistinct"]);
  let sumMCVFreqs = 0;

  for (let i = 1; i <= 5; i++) {
    const stakindStr = "stakind" + i;
    const stavaluesStr = "stavalues" + i;
    const stanumbersStr = "stanumbers" + i;
    const stakind = stats[stakindStr];
    const stavalues = JSON.parse(stats[stavaluesStr]["data"]);
    const stanumbers = JSON.parse(stats[stanumbersStr]["data"]);
    if (stakind > 0) {
      switch (stakind) {
        case 1:
          // Most Common Values
          let valuesTable = (
            <div className="inset-table">
              <Table striped bordered hover size="sm" variant="dark">
                <thead>
                  <tr>
                    <th>Value</th>
                    <th>Frequency</th>
                  </tr>
                </thead>
                <tbody>
                  {stavalues.map((v, i) => (
                    <tr key={"value" + i}>
                      <th>{JSON.stringify(v)}</th>
                      <th>{stanumbers[i]}</th>
                    </tr>
                  ))}
                </tbody>
              </Table>
            </div>
          );
          sumMCVFreqs = stanumbers.reduce(
            (accumulator, currentValue) => accumulator + currentValue,
            0
          );
          cleanStats.set("Most Common Values", [valuesTable, stakindStr]);
          break;
        case 2:
          const yValue = (1 - sumMCVFreqs) / (stavalues.length - 1);
          const yDataValues = [];
          for (const _ in stavalues) {
            yDataValues.push(1);
          }
          let histogram = (
            <HistogramWrapper
              width={500}
              height={200}
              xData={stavalues}
              yData={yDataValues}
              yValue={yValue}
              idx={idx}
              histoModalState={histoModalState}
              showHistoModal={showHistoModal}
              hideHistoModal={hideHistoModal}
            />
          );
          cleanStats.set("Value Histogram (excluding Most Common Values)", [
            histogram,
            stakindStr,
          ]);
          break;
        case 3:
          cleanStats.set("Correlation", [stanumbers[0], stakindStr]);
          break;
        case 4:
          let elementsTable = (
            <Table striped bordered hover size="sm" variant="dark">
              <thead>
                <tr>
                  <th>Element</th>
                  <th>Frequency</th>
                </tr>
              </thead>
              <tbody>
                {stavalues.map((v, i) => (
                  <tr key={"value" + i}>
                    <th>{JSON.stringify(v)}</th>
                    <th>{stanumbers[i]}</th>
                  </tr>
                ))}
              </tbody>
            </Table>
          );
          cleanStats.set("Most Common Elements", [elementsTable, stakindStr]);
          break;
        case 5:
          let counts = {};
          for (let i = 0; i < stanumbers.length - 1; i++) {
            const val = stanumbers[i];
            counts[val] = counts[val] ? counts[val] + 1 : 1;
          }
          const xData = Object.keys(counts).map((s) => JSON.parse(s));
          let tmp = Object.values(counts).map((n) => n / 100);
          tmp[tmp.length - 2] += tmp[tmp.length - 1];
          tmp = tmp.slice(0, tmp.length - 1);
          const maxY = Math.max(...tmp);
          const yData = tmp.map((n) => n / maxY);

          let distinctHistogram = (
            <HistogramWrapper
              width={500}
              height={200}
              xData={xData}
              yData={yData}
              yValue={maxY}
              idx={idx}
              histoModalState={histoModalState}
              showHistoModal={showHistoModal}
              hideHistoModal={hideHistoModal}
            />
          );
          cleanStats.set("Distinct Elements Histogram", [
            distinctHistogram,
            stakindStr,
          ]);
          break;
        default:
          break;
      }
    }
  }
  return cleanStats;
}

export default sanitizeStats;

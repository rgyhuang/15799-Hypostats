import Histogram from "./histogram";
import { Modal, Button } from "react-bootstrap";
import "./histogram.css";

export default function HistogramWrapper({
  width,
  height,
  xData,
  yData,
  yValue,
  idx,
  histoModalState,
  showHistoModal,
  hideHistoModal,
}) {
  return (
    <div>
      <Histogram
        width={width}
        height={height}
        xData={xData}
        yData={yData}
        yValue={yValue}
        small={true}
      />
      {xData.length > 10 && (
        <Button
          onClick={showHistoModal}
          style={{
            border: "none",
            fontSize: "12px",
            margin: "auto",
            display: "block",
            marginTop: "-10px",
            marginBottom: "5px",
          }}
        >
          Open Large View
        </Button>
      )}
      <Modal
        show={histoModalState[idx]}
        onHide={hideHistoModal}
        dialogClassName="modal-90w"
        data-bs-theme="dark"
      >
        <Modal.Header closeButton>
          <Modal.Title>Histogram</Modal.Title>
        </Modal.Header>
        <Modal.Body>
          <Histogram
            width={2500}
            height={600}
            xData={xData}
            yData={yData}
            yValue={yValue}
            small={false}
          />
        </Modal.Body>
      </Modal>
    </div>
  );
}

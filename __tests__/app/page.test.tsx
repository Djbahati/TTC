import React from "react";
import { render, screen, fireEvent } from "@testing-library/react";
import TacticalDashboard from "@/app/page";

jest.mock("@/app/command-center/page", () => {
  return function MockCommandCenter() {
    return <div data-testid="command-center">Command Center</div>;
  };
});
jest.mock("@/app/agent-network/page", () => {
  return function MockAgentNetwork() {
    return <div data-testid="agent-network">Agent Network</div>;
  };
});
jest.mock("@/app/operations/page", () => {
  return function MockOperations() {
    return <div data-testid="operations">Operations</div>;
  };
});
jest.mock("@/app/intelligence/page", () => {
  return function MockIntelligence() {
    return <div data-testid="intelligence">Intelligence</div>;
  };
});
jest.mock("@/app/systems/page", () => {
  return function MockSystems() {
    return <div data-testid="systems">Systems</div>;
  };
});

describe("TacticalDashboard (main page)", () => {
  it("renders the sidebar with navigation items", () => {
    render(<TacticalDashboard />);
    expect(screen.getByText("COMMAND CENTER")).toBeInTheDocument();
    expect(screen.getByText("AGENT NETWORK")).toBeInTheDocument();
    expect(screen.getByText("OPERATIONS")).toBeInTheDocument();
    expect(screen.getByText("INTELLIGENCE")).toBeInTheDocument();
    expect(screen.getByText("SYSTEMS")).toBeInTheDocument();
  });

  it("renders the TACTICAL OPS title", () => {
    render(<TacticalDashboard />);
    expect(screen.getByText("TACTICAL OPS")).toBeInTheDocument();
  });

  it("shows the command center by default (overview section)", () => {
    render(<TacticalDashboard />);
    expect(screen.getByTestId("command-center")).toBeInTheDocument();
  });

  it("navigates to agent network when clicked", () => {
    render(<TacticalDashboard />);
    fireEvent.click(screen.getByText("AGENT NETWORK"));
    expect(screen.getByTestId("agent-network")).toBeInTheDocument();
    expect(screen.queryByTestId("command-center")).not.toBeInTheDocument();
  });

  it("navigates to operations when clicked", () => {
    render(<TacticalDashboard />);
    fireEvent.click(screen.getByText("OPERATIONS"));
    expect(screen.getByTestId("operations")).toBeInTheDocument();
  });

  it("navigates to intelligence when clicked", () => {
    render(<TacticalDashboard />);
    fireEvent.click(screen.getByText("INTELLIGENCE"));
    expect(screen.getByTestId("intelligence")).toBeInTheDocument();
  });

  it("navigates to systems when clicked", () => {
    render(<TacticalDashboard />);
    fireEvent.click(screen.getByText("SYSTEMS"));
    expect(screen.getByTestId("systems")).toBeInTheDocument();
  });

  it("shows system status info in sidebar", () => {
    render(<TacticalDashboard />);
    expect(screen.getByText("SYSTEM ONLINE")).toBeInTheDocument();
    expect(screen.getByText(/AGENTS: 847 ACTIVE/)).toBeInTheDocument();
  });

  it("shows the top toolbar breadcrumb", () => {
    render(<TacticalDashboard />);
    expect(screen.getByText("OVERVIEW")).toBeInTheDocument();
  });
});

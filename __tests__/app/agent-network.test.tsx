import React from "react";
import { render, screen, fireEvent } from "@testing-library/react";
import AgentNetworkPage from "@/app/agent-network/page";

describe("AgentNetworkPage", () => {
  it("renders the page header", () => {
    render(<AgentNetworkPage />);
    expect(screen.getByText("AGENT NETWORK")).toBeInTheDocument();
    expect(
      screen.getByText("Manage and monitor field operatives")
    ).toBeInTheDocument();
  });

  it("renders all agents in the roster", () => {
    render(<AgentNetworkPage />);
    expect(screen.getByText("VENGEFUL SPIRIT")).toBeInTheDocument();
    expect(screen.getByText("OBSIDIAN SENTINEL")).toBeInTheDocument();
    expect(screen.getByText("GHOSTLY FURY")).toBeInTheDocument();
    expect(screen.getByText("CURSED REVENANT")).toBeInTheDocument();
    expect(screen.getByText("VENOMOUS SHADE")).toBeInTheDocument();
    expect(screen.getByText("MYSTIC ENIGMA")).toBeInTheDocument();
    expect(screen.getByText("WRAITH AVENGER")).toBeInTheDocument();
    expect(screen.getByText("SPECTRAL FURY")).toBeInTheDocument();
  });

  it("filters agents by name when searching", () => {
    render(<AgentNetworkPage />);
    const searchInput = screen.getByPlaceholderText("Search agents...");

    fireEvent.change(searchInput, { target: { value: "VENGEFUL" } });

    expect(screen.getByText("VENGEFUL SPIRIT")).toBeInTheDocument();
    expect(screen.queryByText("OBSIDIAN SENTINEL")).not.toBeInTheDocument();
    expect(screen.queryByText("GHOSTLY FURY")).not.toBeInTheDocument();
  });

  it("filters agents by ID when searching", () => {
    render(<AgentNetworkPage />);
    const searchInput = screen.getByPlaceholderText("Search agents...");

    fireEvent.change(searchInput, { target: { value: "G-081Z" } });

    expect(screen.getByText("CURSED REVENANT")).toBeInTheDocument();
    expect(screen.queryByText("VENGEFUL SPIRIT")).not.toBeInTheDocument();
  });

  it("is case-insensitive when filtering", () => {
    render(<AgentNetworkPage />);
    const searchInput = screen.getByPlaceholderText("Search agents...");

    fireEvent.change(searchInput, { target: { value: "vengeful" } });

    expect(screen.getByText("VENGEFUL SPIRIT")).toBeInTheDocument();
  });

  it("shows no agents when search term matches nothing", () => {
    render(<AgentNetworkPage />);
    const searchInput = screen.getByPlaceholderText("Search agents...");

    fireEvent.change(searchInput, { target: { value: "NONEXISTENT" } });

    expect(screen.queryByText("VENGEFUL SPIRIT")).not.toBeInTheDocument();
    expect(screen.queryByText("OBSIDIAN SENTINEL")).not.toBeInTheDocument();
  });

  it("shows agent detail modal when agent row is clicked", () => {
    render(<AgentNetworkPage />);

    const agentRow = screen.getByText("VENGEFUL SPIRIT").closest("tr");
    if (agentRow) fireEvent.click(agentRow);

    expect(screen.getByText("Assign Mission")).toBeInTheDocument();
    expect(screen.getByText("View History")).toBeInTheDocument();
    expect(screen.getByText("Send Message")).toBeInTheDocument();
  });

  it("closes the agent detail modal", () => {
    render(<AgentNetworkPage />);

    const agentRow = screen.getByText("VENGEFUL SPIRIT").closest("tr");
    if (agentRow) fireEvent.click(agentRow);

    expect(screen.getByText("Assign Mission")).toBeInTheDocument();

    const closeButton = screen.getByRole("button", { name: /✕/ });
    fireEvent.click(closeButton);

    expect(screen.queryByText("Assign Mission")).not.toBeInTheDocument();
  });

  it("displays stat cards", () => {
    render(<AgentNetworkPage />);
    expect(screen.getByText("ACTIVE AGENTS")).toBeInTheDocument();
    expect(screen.getByText("COMPROMISED")).toBeInTheDocument();
    expect(screen.getByText("IN TRAINING")).toBeInTheDocument();
  });

  it("renders the Deploy Agent button", () => {
    render(<AgentNetworkPage />);
    expect(screen.getByText("Deploy Agent")).toBeInTheDocument();
  });

  it("renders the table headers", () => {
    render(<AgentNetworkPage />);
    expect(screen.getByText("AGENT ID")).toBeInTheDocument();
    expect(screen.getByText("CODENAME")).toBeInTheDocument();
    expect(screen.getByText("STATUS")).toBeInTheDocument();
    expect(screen.getByText("LOCATION")).toBeInTheDocument();
    expect(screen.getByText("LAST SEEN")).toBeInTheDocument();
    expect(screen.getByText("MISSIONS")).toBeInTheDocument();
    expect(screen.getByText("RISK")).toBeInTheDocument();
    expect(screen.getByText("ACTIONS")).toBeInTheDocument();
  });
});

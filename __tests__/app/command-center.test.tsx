import React from "react";
import { render, screen } from "@testing-library/react";
import CommandCenterPage from "@/app/command-center/page";

describe("CommandCenterPage", () => {
  it("renders the agent allocation section", () => {
    render(<CommandCenterPage />);
    expect(screen.getByText("AGENT ALLOCATION")).toBeInTheDocument();
  });

  it("displays agent count statistics", () => {
    render(<CommandCenterPage />);
    expect(screen.getAllByText("190").length).toBeGreaterThanOrEqual(1);
    expect(screen.getByText("990")).toBeInTheDocument();
    expect(screen.getByText("290")).toBeInTheDocument();
  });

  it("displays agent categories", () => {
    render(<CommandCenterPage />);
    expect(screen.getByText("Active Field")).toBeInTheDocument();
    expect(screen.getByText("Undercover")).toBeInTheDocument();
    expect(screen.getByText("Training")).toBeInTheDocument();
  });

  it("renders the activity log section", () => {
    render(<CommandCenterPage />);
    expect(screen.getByText("ACTIVITY LOG")).toBeInTheDocument();
  });

  it("renders agent entries in the allocation list", () => {
    render(<CommandCenterPage />);
    expect(screen.getByText("VENGEFUL SPIRIT")).toBeInTheDocument();
    expect(screen.getByText("OBSIDIAN SENTINEL")).toBeInTheDocument();
    expect(screen.getByText("GHOSTLY FURY")).toBeInTheDocument();
    expect(screen.getByText("CURSED REVENANT")).toBeInTheDocument();
  });

  it("renders agent IDs", () => {
    render(<CommandCenterPage />);
    expect(screen.getByText("G-078W")).toBeInTheDocument();
    expect(screen.getByText("G-079X")).toBeInTheDocument();
    expect(screen.getByText("G-080Y")).toBeInTheDocument();
    expect(screen.getByText("G-081Z")).toBeInTheDocument();
  });

  it("renders the encrypted chat activity section", () => {
    render(<CommandCenterPage />);
    expect(screen.getByText("ENCRYPTED CHAT ACTIVITY")).toBeInTheDocument();
  });

  it("renders the mission activity overview section", () => {
    render(<CommandCenterPage />);
    expect(screen.getByText("MISSION ACTIVITY OVERVIEW")).toBeInTheDocument();
  });

  it("renders the mission information section", () => {
    render(<CommandCenterPage />);
    expect(screen.getByText("MISSION INFORMATION")).toBeInTheDocument();
  });

  it("displays successful and failed mission categories", () => {
    render(<CommandCenterPage />);
    expect(screen.getByText("Successful Missions")).toBeInTheDocument();
    expect(screen.getByText("Failed Missions")).toBeInTheDocument();
  });

  it("renders activity log entries with agent names", () => {
    render(<CommandCenterPage />);
    expect(screen.getByText("gh0st_Fire")).toBeInTheDocument();
    expect(screen.getByText("dr4g0n_V3in")).toBeInTheDocument();
  });

  it("displays activity log locations", () => {
    render(<CommandCenterPage />);
    expect(screen.getByText("Berlin")).toBeInTheDocument();
    expect(screen.getByText("Cairo")).toBeInTheDocument();
    expect(screen.getByText("Havana")).toBeInTheDocument();
  });
});
